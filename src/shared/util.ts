

export function relativePath(root: string, cur: string) {
  return cur.replace(root, "");
}

/**
 * 
 * @param func 
 * @param wait 
 * @returns 
 */
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  delay: number = 500,
): (...args: Parameters<T>) => void {
  let timeout: number | null = null;

  return function (...args: Parameters<T>) {
    if (timeout) {
      clearTimeout(timeout);
    }
    timeout = setTimeout(() => {
      func(...args);
    }, delay);
  };
}

