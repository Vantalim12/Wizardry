import { execSync } from "child_process";
import * as dotenv from "dotenv";
import * as path from "path";

// Load environment variables
dotenv.config();

async function main() {
  console.log("Starting pool creation process...");

  const rustBinaryPath = path.join(
    __dirname,
    "../../rust/target/release/create_pool"
  );

  try {
    // Run the Rust binary
    execSync(rustBinaryPath, {
      stdio: "inherit",
      env: process.env,
    });

    console.log("Pool creation completed successfully");
  } catch (error) {
    console.error("Pool creation failed:", error);
    process.exit(1);
  }
}

main();
