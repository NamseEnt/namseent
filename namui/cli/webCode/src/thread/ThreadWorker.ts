import { threadMain, type ThreadStartSupplies } from "./threadMain";

self.onmessage = async (message) => {
    threadMain(message.data as ThreadStartSupplies);
};
