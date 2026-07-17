import { addGold } from "@/shared/api";

type AddGoldParams = {
  player?: number;
  amount?: number;
};

export async function submitAddGold({ player = 0, amount = 10 }: AddGoldParams = {}) {
  return addGold.call({ player, amount });
}
