/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum NodeKeys {
  LEFT = 0,
  RIGHT = 1,
  MIDDLE = 2,
  WHEEL = 3,
  X = 4,
  X2 = 5,
  UP = 6,
  DOWN = 7,
  VERTICAL = 8,
  HORIZONTAL = 9,
}
export interface MousePosition {
  x: number
  y: number
}
export const enum MouseButton {
  Left = 0,
  Right = 1,
  Middle = 2,
}
export function moveTo(x: number, y: number): boolean
export function press(button: NodeKeys): boolean
export function release(button: NodeKeys): boolean
export function getPosition(): MousePosition | null
export function wheel(delta: number): boolean
export function scroll(delta: number): boolean
export function click(key: NodeKeys): boolean