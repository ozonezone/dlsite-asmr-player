// States for protected routes
import { atom } from "jotai";

export type PlayerData = {
  queue: {
    productId: string;
    path: string[];
  }[];
  queueIdx: number;
  playing: boolean;
} | null;
export const playerDataAtom = atom<PlayerData>(null);
