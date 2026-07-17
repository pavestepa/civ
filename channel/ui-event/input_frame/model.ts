export const OP = "InputFrame";

export type InputFrame = {
  keys: string[];
  scroll_delta: number;
  mouse_click?: boolean;
  mouse_x?: number;
  mouse_y?: number;
};
