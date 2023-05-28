// States for protected routes
import { atom } from "jotai";

export type PlayerData = {
  productId: string;
  queue: {
    title: string;
    path: string[];
  }[];
  queueIdx: number;
  playing: boolean;
} | null;
export const playerDataAtom = atom<PlayerData>(null);
