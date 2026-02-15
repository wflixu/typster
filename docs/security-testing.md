# 文件操作安全性测试文档

本文档描述了 Typster 项目中实施的文件路径安全验证措施，以及如何测试这些安全功能。

## 安全功能概述

### 1. 前端安全验证 (TypeScript)

**位置**: `src/utils/path-security.ts`

**核心功能**:
- ✅ 路径穿越检测
- ✅ 路径规范化
- ✅ 文件类型白名单验证
- ✅ 项目范围验证

**应用位置**:
- `src/pages/home/Sidebar.vue` - 文件创建、重命名、删除
- `src/pages/typst/TypstEditor.vue` - 文件读取、保存

### 2. 后端安全验证 (Rust)

**位置**: `src-tauri/src/cmds/fs.rs`

**核心功能**:
- ✅ 路径穿越模式检测
- ✅ 系统目录访问阻止
- ✅ 所有文件操作 API 的统一验证

**保护的 API**:
- `fs_read_file_text`
- `fs_write_file_text`
- `fs_read_file_binary`
- `fs_write_file_binary`
- `fs_create_file`
- `fs_list_dir`

## 防护的攻击类型

### 1. 路径穿越攻击 (Path Traversal)

**攻击示例**:
```
../../../etc/passwd
..\\..\\..\\windows\\system32\\config\\sam
%2e%2e%2fetc%2fpasswd
```

**防护机制**:
- 前端: `hasPathTraversalPattern()` 检测常见模式
- 后端: `has_path_traversal()` 双重验证

### 2. 系统目录访问

**阻止的路径**:
- `/etc/` - Linux 系统配置
- `/sys/` - Linux 内核接口
- `/proc/` - Linux 进程信息
- `/dev/` - Linux 设备文件
- `/root/` - Linux 超级用户目录
- `~/.ssh/` - SSH 密钥
- `~/.gnupg/` - GPG 密钥

### 3. 文件类型限制

**允许的文件类型**:
- `.typ` - Typst 源文件
- `.md` - Markdown 文档
- `.txt` - 纯文本
- `.toml` - 配置文件
- `.json` - JSON 配置
- `.svg`, `.png`, `.jpg`, `.jpeg` - 图片
- `.pdf` - PDF 文档

## 测试用例

### 手动测试步骤

#### 测试 1: 路径穿越攻击 - 读取文件

**预期结果**: 拒绝访问并记录安全警告

```javascript
// 在浏览器控制台测试
const maliciousPath = '../../../etc/passwd';
// 应该被阻止并显示: "安全警告: 检测到路径穿越攻击尝试"
```

#### 测试 2: URL 编码的路径穿越

**预期结果**: 拒绝访问

```javascript
const encodedPath = '%2e%2e%2fetc%2fpasswd';
// 应该被阻止
```

#### 测试 3: 系统目录访问

**预期结果**: 拒绝访问

```javascript
const systemPath = '/etc/shadow';
// 应该被阻止并显示: "安全警告: 文件路径必须在项目范围内"
```

#### 测试 4: 混合编码攻击

**预期结果**: 拒绝访问

```javascript
const mixedPath = '..%2fetc%2fpasswd';
// 应该被阻止
```

#### 测试 5: 正常文件操作（正向测试）

**预期结果**: 允许访问

```javascript
const projectPath = '/Users/username/project/main.typ';
// 应该正常工作
```

#### 测试 6: 项目边界验证

**预期结果**: 拒绝项目外的文件访问

```javascript
const outsidePath = '/tmp/evil.txt';
const projectPath = '/Users/username/project';
// 应该被阻止
```

### 自动化测试示例

创建 `src/utils/__tests__/path-security.test.ts`:

```typescript
import {
  hasPathTraversalPattern,
  sanitizePath,
  validateTypstFilePath,
  isAllowedFileType
} from '../path-security';

describe('Path Security Tests', () => {
  describe('hasPathTraversalPattern', () => {
    it('should detect ../ pattern', () => {
      expect(hasPathTraversalPattern('../../../etc/passwd')).toBe(true);
    });

    it('should detect URL encoded pattern', () => {
      expect(hasPathTraversalPattern('%2e%2e%2fetc%2fpasswd')).toBe(true);
    });

    it('should allow safe paths', () => {
      expect(hasPathTraversalPattern('/home/user/project/file.typ')).toBe(false);
    });
  });

  describe('validateTypstFilePath', () => {
    const projectPath = '/home/user/project';

    it('should allow valid project files', () => {
      expect(validateTypstFilePath(`${projectPath}/main.typ`, projectPath)).toBe(true);
    });

    it('should reject path traversal attempts', () => {
      expect(validateTypstFilePath(`${projectPath}/../etc/passwd`, projectPath)).toBe(false);
    });

    it('should reject files outside project', () => {
      expect(validateTypstFilePath('/tmp/file.typ', projectPath)).toBe(false);
    });
  });

  describe('isAllowedFileType', () => {
    const allowedTypes = ['typ', 'md', 'txt', 'toml'];

    it('should allow whitelisted types', () => {
      expect(isAllowedFileType('file.typ', allowedTypes)).toBe(true);
      expect(isAllowedFileType('file.md', allowedTypes)).toBe(true);
    });

    it('should reject non-whitelisted types', () => {
      expect(isAllowedFileType('file.exe', allowedTypes)).toBe(false);
      expect(isAllowedFileType('file.sh', allowedTypes)).toBe(false);
    });
  });
});
```

## 安全检查清单

### 开发阶段
- [ ] 所有文件操作都使用验证函数
- [ ] 路径穿越检查在前后端都实施
- [ ] 文件类型白名单强制执行
- [ ] 项目范围边界验证

### 测试阶段
- [ ] 所有已知的路径穿越模式都被阻止
- [ ] 正常文件操作不受影响
- [ ] 错误消息清晰且不泄露系统信息
- [ ] 安全事件被正确记录

### 部署阶段
- [ ] 代码审查确认所有安全措施
- [ ] 安全测试通过
- [ ] 文档更新
- [ ] 已知的安全限制已记录

## 安全事件响应

如果检测到路径穿越攻击：

1. **前端**:
   - 立即阻止操作
   - 记录到控制台: `console.error('安全警告: 检测到路径穿越攻击尝试', path)`
   - 显示用户友好的错误消息

2. **后端**:
   - 返回 `PermissionDenied` 错误
   - 不执行任何文件操作
   - 记录安全事件（建议添加日志系统）

## 已知限制

1. **相对路径**: 当前实现假设使用绝对路径
2. **符号链接**: 未验证符号链接目标
3. **竞态条件**: TOCTOU (Time-of-check to Time-of-use) 问题可能存在

## 未来改进

1. 添加路径验证缓存
2. 实现更严格的符号链接检查
3. 添加文件大小限制
4. 实现文件内容扫描（恶意代码检测）
5. 添加安全审计日志

## 参考

- [OWASP Path Traversal](https://owasp.org/www-community/attacks/Path_Traversal)
- [CWE-22: Improper Limitation of a Pathname to a Restricted Directory](https://cwe.mitre.org/data/definitions/22.html)
- [Rust Security Guidelines](https://doc.rust-lang.org/nomicon/safe-code-gidelines.html)
