import { execSync } from "child_process";
import * as dotenv from "dotenv";
import * as path from "path";

// Load environment variables
dotenv.config();

async function main() {
  console.log("Starting token distribution process...");

  const rustBinaryPath = path.join(
    __dirname,
    "../../rust/target/release/distribute_tokens"
  );

  try {
    // Run the Rust binary
    execSync(rustBinaryPath, {
      stdio: "inherit",
      env: process.env,
    });

    console.log("Token distribution completed successfully");
  } catch (error) {
    console.error("Token distribution failed:", error);
    process.exit(1);
  }
}

main();
