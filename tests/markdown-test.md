# Markdown 语法测试文档

这是一个完整的 Markdown 语法测试文档，用于测试编辑器的各项功能。

# 1. 标题层级测试

# 一级标题 (H1)1

## 二级标题 (H2)

### 三级标题 (H3)

#### 四级标题 (H4)

##### 五级标题 (H5)

###### 六级标题 (H6)

## 2. 文本格式测试

**粗体文本** (使用双星号)

**粗体文本** (使用双下划线)

*斜体文本* (使用单星号)

*斜体文本* (使用单下划线)

***粗斜体文本*** (三颗星号)

***粗斜体文本*** (三条下划线)

**粗体和 *斜体* 组合**

~~删除线文本~~

==高亮文本== (如果支持)

## 3. 列表测试

### 无序列表

- 第一项
- 第二项
- 第三项
  - 嵌套项目 1
  - 嵌套项目 2
    - 深层嵌套 1
    - 深层嵌套 2
- 第四项

### 有序列表

1. 第一项
2. 第二项
3. 第三项
  1. 嵌套有序项 1
  2. 嵌套有序项 2
    1. 深层嵌套 1
    2. 深层嵌套 2
4. 第四项

### 任务列表

- [x] 已完成的任务

未完成的任务

- [ ] 另一个待办事项
  - [x] 子任务已完成
  - [ ] 子任务未完成

## 4. 链接和图片 sddddd

### 链接测试

[行内链接](https://www.example.com)

[带标题的链接](https://www.example.com)

[相对链接](./other-file.md)

[邮箱链接](mailto:user@example.com)

### 引用式链接

[引用式链接](https://www.google.com)

[另一个引用](https://www.github.com)

### 图片测试

行内图片引用式图片

## 5. 引用测试

> 这是一段引用文本
> 可以跨越多行
>
> 还可以包含其他 **格式化** 内容

### 嵌套引用

> 外层引用
>
> > 内层引用
> >
> > > 更深层的引用
> >
> > 回到内层
>
> 回到外层

## 6. 代码测试

### 行内代码

这是 `行内代码` 示例。

使用反引号包裹代码：`let x = 10;`

### 代码块

#### 使用反引号的代码块

```javascript
// JavaScript 示例
function greetUser(name) {
    console.log(`Hello, ${name}!`);
    return `Welcome, ${name}`;
}

const user = "World";
greetUser(user);
```

```python
# Python 示例
def fibonacci(n):
    if n <= 1:
        return n
    else:
        return fibonacci(n-1) + fibonacci(n-2)

# 计算前10个斐波那契数
for i in range(10):
    print(f"F({i}) = {fibonacci(i)}")
```

#### 带语法高亮的代码块

```typescript
interface User {
    id: number;
    name: string;
    email: string;
    avatar?: string;
}

class UserService {
    private users: User[] = [];

    addUser(user: User): void {
        this.users.push(user);
    }

    getUserById(id: number): User | undefined {
        return this.users.find(u => u.id === id);
    }
}
```

#### 缩进式代码块

```
<!DOCTYPE html>
<html>
<head>
    <title>HTML 示例</title>
</head>
<body>
    <h1>Hello World</h1>
</body>
</html>
```

## 7. 分隔线测试

---

---

---

---

## 8. 表格测试

### 基本表格

### 对齐表格

### 复杂表格

## 9. HTML 支持

### HTML 标签测试

这是 **HTML strong 标签** 和 *HTML em 标签*。

Ctrl + S 保存文件

红色文本

这是一个带样式的 div 容器

### HTML 列表

- HTML 无序列表项 1
- HTML 无序列表项 2
- HTML 无序列表项 3

## 10. 脚注和扩展语法

### 脚注测试

这是一个脚注引用示例[^1](这是第一个脚注的内容)。

这里还有另一个脚注[^note](这是一个命名脚注，可以包含更详细的信息)。

### 定义列表

Markdown

一种轻量级标记语言

HTML

超文本标记语言

## 11. 数学公式测试 (如果支持)

行内公式：$E = mc^2$

块级公式：
$$
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
$$

更多数学公式：
$$
\begin{align}
\nabla \times \vec{\mathbf{B}} -\frac{1}{c}\frac{\partial\vec{\mathbf{E}}}{\partial t} &= \frac{4\pi}{c}\vec{\mathbf{j}} 
\nabla \cdot \vec{\mathbf{E}} &= 4 \pi \rho 
\nabla \times \vec{\mathbf{E}}+\frac{1}{c}\frac{\partial\vec{\mathbf{B}}}{\partial t} &= \vec{0} 
\nabla \cdot \vec{\mathbf{B}} &= 0
\end{align}
$$

## 12. 特殊字符和转义

### 特殊字符

&copy; 版权符号
&reg; 注册商标符号
&trade; 商标符号
&hearts; ♥
&clubs; ♣
&spades; ♠
&diams; ♦

### 转义字符测试

 这不是粗体 
 这不是标题 
 这不是链接 
 这不是代码 

## 13. 性能测试

### 大量文本

这是一段很长的文本，用于测试编辑器在处理大量内容时的性能表现。Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

这段文本继续测试编辑器的滚动和渲染性能。Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt.

## 14. 编辑功能测试3 

### 格式切换

**这是粗体** *这是斜体* ~~这是删除线~~ `这是代码`

### 嵌套格式

**粗体中的 *斜体* 文本**

*斜体中的 **粗体** 文本*

### 链接和格式组合

**[粗体链接](https://example.com)**

*[斜体链接](https://example.com)*

## 测试完成

这个文档包含了 Markdown 的主要语法元素，可以用来测试编辑器的各项功能：

- ✅ 标题层级
- ✅ 文本格式化
- ✅ 列表（有序、无序、任务）
- ✅ 链接和图片
- ✅ 引用和嵌套引用
- ✅ 代码块和语法高亮
- ✅ 表格
- ✅ HTML 支持
- ✅ 分隔线
- ✅ 特殊字符和转义
- ✅ 数学公式（如果支持）

---

**最后更新**: 2025年12月6日
**文档用途**: 测试 Markdown 编辑器功能