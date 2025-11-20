// File download URLs - update these with your actual file URLs
const DOWNLOAD_URLS = {
    mac: '/downloads/MsProjectMerger.dmg',
    windows: '/downloads/msproject-merge.exe'
};

// Email collection endpoint - update with your backend API
const EMAIL_API_ENDPOINT = '/api/collect-email';

document.getElementById('downloadForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const email = document.getElementById('emailInput').value;
    const platform = e.submitter.dataset.platform;
    
    if (!email || !platform) {
        return;
    }

    try {
        // Send email to backend (you'll need to implement this endpoint)
        await fetch(EMAIL_API_ENDPOINT, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                email: email,
                platform: platform,
                timestamp: new Date().toISOString()
            })
        });

        // Show success message
        document.getElementById('successMessage').style.display = 'block';
        
        // Trigger download
        const downloadUrl = DOWNLOAD_URLS[platform];
        const link = document.createElement('a');
        link.href = downloadUrl;
        link.download = '';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);

        // Clear form
        document.getElementById('emailInput').value = '';
        
        // Hide success message after 5 seconds
        setTimeout(() => {
            document.getElementById('successMessage').style.display = 'none';
        }, 5000);

    } catch (error) {
        console.error('Error:', error);
        alert('An error occurred. Please try again.');
    }
});
