export const capitalize = (text: string, denominator: string) => text.split(denominator).map(v => v.split('')).map(v => v.map((v, i) => i === 0 ? v.toUpperCase() : v)).map(v => v.join('')).join(' ')
