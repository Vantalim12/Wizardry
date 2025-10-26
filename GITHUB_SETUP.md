# GitHub Repository Setup

## Commands to Push to GitHub

Run these commands to create and push to GitHub:

### 1. Create a new GitHub repository

Go to https://github.com/new and create a **private** repository named:

- `wizardry` or `wiz-sachi-distributor`

**Important**: Make it **PRIVATE** to keep your code and configuration safe.

### 2. Add the remote and push

After creating the repository, run:

```bash
# Add your GitHub repository (replace with your actual repo URL)
git remote add origin https://github.com/YOUR_USERNAME/wizardry.git

# Push to GitHub
git branch -M main
git push -u origin main
```

### 3. Alternative: Using SSH (if you have SSH keys set up)

```bash
git remote add origin git@github.com:YOUR_USERNAME/wizardry.git
git branch -M main
git push -u origin main
```

## What's Protected

Your `.gitignore` is configured to protect:

- ✅ Keypairs (`keypairs/*.json`)
- ✅ Environment files (`.env`)
- ✅ Build artifacts
- ✅ Logs
- ✅ Node modules

## After Pushing

1. **Add a repository description**: "Automated WIZ-SACHI token distributor for Solana"
2. **Add topics**: `solana`, `token-distributor`, `meteora`, `jupiter`, `rust`, `typescript`
3. **Update README** if you want to change the clone URL after deployment

## Setting Up Collaborators (Optional)

If you want to add team members:

1. Go to repository Settings → Collaborators
2. Add their GitHub usernames
3. They can clone with: `git clone https://github.com/YOUR_USERNAME/wizardry.git`

## Future Pushes

Once set up, you can push updates with:

```bash
git add .
git commit -m "Your commit message"
git push
```

## ⚠️ Security Reminder

- Never commit `.env` files with real credentials
- Never commit keypairs
- Keep the repository **PRIVATE**
- Use environment variables for all sensitive data
