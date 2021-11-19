import fileSystem from "../../fileSystem/fileSystem";

export async function renameSequence(oldTitle: string, newTItle: string) {
  await fileSystem.rename(
    `/sequence/${oldTitle}.json`,
    `/sequence/${newTItle}.json`,
  );
  return;
}
