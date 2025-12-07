/**
 * 系统总线 - 纯静态的 EventBus
 * 用于应用内组件间的通信和解耦
 *
 * 使用方式：
 * import EventBus from '@/shared/EventBus';
 *
 * // 监听事件
 * EventBus.on('save', (data) => { ... });
 *
 * // 触发事件
 * EventBus.emit('save', { ... });
 */

// 事件监听器类型定义
type EventListener<T = any> = (data: T) => void;

// 事件监听器存储接口
interface IListenerStore {
    [eventName: string]: Set<EventListener>;
}

/**
 * EventBus - 纯静态方法，全局单例
 */
export class EventBus {
    // 静态事件监听器存储 - 全局唯一
    private static listeners: IListenerStore = {};
    private static debugMode: boolean = import.meta.env.DEV;

    /**
     * 注册事件监听器
     * @param eventName 事件名称
     * @param listener 监听器函数
     * @param once 是否只监听一次（可选，默认 false）
     * @returns 移除监听器的函数
     */
    public static on<T = any>(
        eventName: string,
        listener: EventListener<T>,
        once: boolean = false
    ): () => void {
        if (!EventBus.listeners[eventName]) {
            EventBus.listeners[eventName] = new Set();
        }

        // 创建包装器处理 once 选项
        let wrapper: EventListener<T>;
        if (once) {
            wrapper = (data: T) => {
                listener(data);
                EventBus.off(eventName, wrapper);
            };
        } else {
            wrapper = listener;
        }

        EventBus.listeners[eventName].add(wrapper);
        EventBus.log(`Listener added for event: ${eventName}, once: ${once}`);

        // 返回移除监听器的函数
        return () => EventBus.off(eventName, wrapper);
    }

    /**
     * 注册一次性事件监听器
     * @param eventName 事件名称
     * @param listener 监听器函数
     * @returns 移除监听器的函数
     */
    public static once<T = any>(
        eventName: string,
        listener: EventListener<T>
    ): () => void {
        return EventBus.on(eventName, listener, true);
    }

    /**
     * 移除事件监听器
     * @param eventName 事件名称
     * @param listener 监听器函数（可选，如果不提供则移除所有该事件的监听器）
     */
    public static off(eventName: string, listener?: EventListener): void {
        if (!EventBus.listeners[eventName]) {
            return;
        }

        if (listener) {
            // 移除特定的监听器
            EventBus.listeners[eventName].delete(listener);
            EventBus.log(`Listener removed for event: ${eventName}`);

            // 如果没有监听器了，删除该事件
            if (EventBus.listeners[eventName].size === 0) {
                delete EventBus.listeners[eventName];
            }
        } else {
            // 移除所有该事件的监听器
            delete EventBus.listeners[eventName];
            EventBus.log(`All listeners removed for event: ${eventName}`);
        }
    }

    /**
     * 触发事件
     * @param eventName 事件名称
     * @param data 事件数据（可选）
     * @param immediate 是否同步触发（可选，默认 true）
     */
    public static emit<T = any>(
        eventName: string,
        data?: T,
        immediate: boolean = true
    ): void {
        EventBus.log(`Event emitted: ${eventName}`, data);

        const listeners = EventBus.listeners[eventName];
        if (!listeners || listeners.size === 0) {
            EventBus.log(`No listeners for event: ${eventName}`);
            return;
        }

        const executeListeners = () => {
            // 创建监听器数组的副本，避免在执行过程中监听器被修改导致的问题
            const listenersArray = Array.from(listeners);
            listenersArray.forEach((listener, index) => {
                try {
                    listener(data!);
                } catch (error) {
                    console.error(
                        `Error in event listener ${index} for event "${eventName}":`,
                        error
                    );
                }
            });
        };

        if (immediate) {
            executeListeners();
        } else {
            // 使用 setTimeout 实现异步执行
            setTimeout(executeListeners, 0);
        }
    }

    /**
     * 异步触发事件
     * @param eventName 事件名称
     * @param data 事件数据（可选）
     */
    public static emitAsync<T = any>(eventName: string, data?: T): Promise<void> {
        return new Promise((resolve) => {
            EventBus.once(eventName, () => {
                resolve();
            });

            EventBus.emit(eventName, data, false);
        });
    }

    /**
     * 等待事件被触发
     * @param eventName 事件名称
     * @param timeout 超时时间（毫秒，可选）
     * @returns Promise，当事件触发时解析
     */
    public static waitFor<T = any>(
        eventName: string,
        timeout?: number
    ): Promise<T> {
        return new Promise((resolve, reject) => {
            let timeoutId: number | undefined;

            const listener = EventBus.once<T>(eventName, (data) => {
                if (timeoutId) {
                    clearTimeout(timeoutId);
                }
                resolve(data);
            });

            if (timeout) {
                timeoutId = window.setTimeout(() => {
                    EventBus.off(eventName, listener);
                    reject(new Error(`Event "${eventName}" timeout after ${timeout}ms`));
                }, timeout);
            }
        });
    }

    /**
     * 获取事件的所有监听器数量
     * @param eventName 事件名称（可选，如果不提供则返回所有事件的监听器总数）
     * @returns 监听器数量
     */
    public static listenerCount(eventName?: string): number {
        if (eventName) {
            return EventBus.listeners[eventName]?.size || 0;
        } else {
            return Object.values(EventBus.listeners).reduce(
                (total, listeners) => total + listeners.size,
                0
            );
        }
    }

    /**
     * 获取所有已注册的事件名称
     * @returns 事件名称数组
     */
    public static eventNames(): string[] {
        return Object.keys(EventBus.listeners);
    }

    /**
     * 清除所有事件监听器
     */
    public static clear(): void {
        EventBus.listeners = {};
        EventBus.log('All event listeners cleared');
    }

    /**
     * 清除特定事件的所有监听器
     * @param eventName 事件名称
     */
    public static clearEvent(eventName: string): void {
        delete EventBus.listeners[eventName];
        EventBus.log(`Event "${eventName}" cleared`);
    }

    /**
     * 检查是否有指定事件的监听器
     * @param eventName 事件名称
     * @returns 是否有监听器
     */
    public static hasListeners(eventName: string): boolean {
        return !!(EventBus.listeners[eventName] && EventBus.listeners[eventName].size > 0);
    }

    /**
     * 获取调试信息
     * @returns 调试信息对象
     */
    public static getDebugInfo(): {
        eventCount: number;
        totalListeners: number;
        events: { [eventName: string]: number };
    } {
        const events: { [eventName: string]: number } = {};
        let totalListeners = 0;

        for (const [eventName, listeners] of Object.entries(EventBus.listeners)) {
            events[eventName] = listeners.size;
            totalListeners += listeners.size;
        }

        return {
            eventCount: Object.keys(EventBus.listeners).length,
            totalListeners,
            events,
        };
    }

    /**
     * 打印调试信息
     */
    public static printDebugInfo(): void {
        const debugInfo = EventBus.getDebugInfo();
        console.group('EventBus Debug Info');
        console.log('Total Events:', debugInfo.eventCount);
        console.log('Total Listeners:', debugInfo.totalListeners);
        console.table(debugInfo.events);
        console.groupEnd();
    }

    /**
     * 日志输出
     * @param message 日志消息
     * @param data 可选数据
     */
    private static log(message: string, data?: any): void {
        if (EventBus.debugMode) {
            if (data !== undefined) {
                console.log(`[EventBus] ${message}`, data);
            } else {
                console.log(`[EventBus] ${message}`);
            }
        }
    }
}

// 导出类型定义
export type { EventListener, IListenerStore };
