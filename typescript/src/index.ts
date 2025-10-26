// Main entry point for WIZ-SACHI Distributor
import * as dotenv from "dotenv";

dotenv.config();

console.log("WIZ-SACHI Token Distributor");
console.log("============================");
console.log("");
console.log("Available commands:");
console.log("  npm run collect-fees   - Collect pool fees");
console.log("  npm run swap          - Swap SOL to SACHI");
console.log("  npm run distribute    - Distribute tokens");
console.log("  npm run create-pool   - Create pool (one-time)");
console.log("");
