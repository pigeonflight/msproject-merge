# MS Project Merger - Download Website

A modern landing page for MS Project Merger with email collection before download.

## Features

- 🎨 Modern dark theme with gradient accents
- 📧 Email collection before download
- 🖥️ Platform-specific downloads (macOS .dmg, Windows .exe)
- 📱 Fully responsive design
- ⚡ Serverless API with Vercel

## Deployment to Vercel

### Prerequisites
- A Vercel account (free tier works)
- Your build artifacts (.dmg and .exe files)

### Steps

1. **Prepare your files:**
   ```bash
   cd website
   mkdir public/downloads
   # Copy your build artifacts
   cp ../target/release/MsProjectMerger.dmg public/downloads/
   cp ../target/x86_64-pc-windows-gnu/release/msproject-merge.exe public/downloads/
   ```

2. **Install Vercel CLI (optional):**
   ```bash
   npm install -g vercel
   ```

3. **Deploy:**
   ```bash
   vercel
   ```
   
   Or push to GitHub and connect the repository in the Vercel dashboard.

4. **Configure email storage (choose one):**

   **Option A: Vercel KV (Redis)**
   - Install: `npm install @vercel/kv`
   - Uncomment the KV code in `api/collect-email.js`
   - Add KV storage in Vercel dashboard

   **Option B: Webhook to external service**
   - Set environment variable `WEBHOOK_URL` in Vercel
   - Uncomment webhook code in `api/collect-email.js`
   - Use services like Zapier, Make.com, or n8n

   **Option C: Email service (Mailchimp, SendGrid, etc.)**
   - Install the respective SDK
   - Add API keys as environment variables
   - Implement the integration in `api/collect-email.js`

## File Structure

```
website/
├── index.html          # Main landing page
├── styles.css          # Styling
├── script.js           # Frontend logic
├── vercel.json         # Vercel configuration
├── api/
│   └── collect-email.js # Serverless function for email collection
├── public/
│   └── downloads/      # Place your .dmg and .exe files here
└── README.md
```

## Customization

### Update Download URLs
Edit `script.js` and update the `DOWNLOAD_URLS` object with your actual file paths.

### Modify Styling
Edit `styles.css` to change colors, fonts, or layout. CSS variables are defined at the top for easy theming.

### Add Analytics
Add your analytics code (Google Analytics, Plausible, etc.) to `index.html` before the closing `</body>` tag.

## Local Development

Simply open `index.html` in a browser, or use a local server:

```bash
python3 -m http.server 8000
# Visit http://localhost:8000
```

## Environment Variables (Vercel)

Set these in your Vercel project settings:

- `WEBHOOK_URL` (optional): URL to send email data to
- Any API keys for email services you integrate

## License

Same as MS Project Merger application.
