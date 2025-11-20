// Vercel Serverless Function for email collection
// This will be deployed as an API endpoint at /api/collect-email

export default async function handler(req, res) {
    // Only allow POST requests
    if (req.method !== 'POST') {
        return res.status(405).json({ error: 'Method not allowed' });
    }

    const { email, platform, timestamp } = req.body;

    // Basic validation
    if (!email || !email.includes('@')) {
        return res.status(400).json({ error: 'Invalid email address' });
    }

    try {
        // Option 1: Store in Vercel KV (requires @vercel/kv package)
        // const { kv } = await import('@vercel/kv');
        // await kv.lpush('email-list', JSON.stringify({ email, platform, timestamp }));

        // Option 2: Send to a third-party service (e.g., Mailchimp, SendGrid, etc.)
        // Example with a webhook:
        // await fetch(process.env.WEBHOOK_URL, {
        //     method: 'POST',
        //     headers: { 'Content-Type': 'application/json' },
        //     body: JSON.stringify({ email, platform, timestamp })
        // });

        // Option 3: Log to console (for development/testing)
        console.log('Email collected:', { email, platform, timestamp });

        // For now, just return success
        // You should implement one of the above options for production
        return res.status(200).json({
            success: true,
            message: 'Email collected successfully'
        });

    } catch (error) {
        console.error('Error collecting email:', error);
        return res.status(500).json({ error: 'Internal server error' });
    }
}
