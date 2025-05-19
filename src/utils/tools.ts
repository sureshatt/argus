import { twMerge } from 'tailwind-merge'

export function cn(a: string, b: string): string {
    return twMerge(a, b)
}