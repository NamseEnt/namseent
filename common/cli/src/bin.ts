import { Command } from "commander";
import { createProject } from "./createProject";

const program = new Command();

program
  .command("create <projectName>")
  .description("create project")
  .action(async (projectName) => {
    await createProject(projectName);
  });

program.parse(process.argv);
