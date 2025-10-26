import { execSync } from "child_process";
import * as dotenv from "dotenv";
import * as path from "path";

// Load environment variables
dotenv.config();

async function main() {
  console.log("Starting SOL to SACHI swap process...");

  const rustBinaryPath = path.join(
    __dirname,
    "../../rust/target/release/swap_sol_to_sachi"
  );

  try {
    // Run the Rust binary
    execSync(rustBinaryPath, {
      stdio: "inherit",
      env: process.env,
    });

    console.log("Swap completed successfully");
  } catch (error) {
    console.error("Swap failed:", error);
    process.exit(1);
  }
}

main();
