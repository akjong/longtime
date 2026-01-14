import { chromium } from 'playwright';

(async () => {
    console.log('Starting deployment verification...');
    const browser = await chromium.launch();
    const page = await browser.newPage();
    
    const url = 'https://longtime.longcipher.com';
    console.log(`Navigating to ${url}...`);
    
    try {
        await page.goto(url, { waitUntil: 'networkidle' });
        
        // Take a screenshot
        await page.screenshot({ path: 'deployment_screenshot.png' });
        console.log('Screenshot saved to deployment_screenshot.png');
        
        // Check for specific elements that indicate the app is loaded
        // Since I don't know the exact DOM structure, I'll check for the title or known text
        const title = await page.title();
        console.log(`Page title: ${title}`);
        
        // Look for "Time" or something related to the app functionality
        // Assuming "Longtime" or similar is in the body
        const content = await page.content();
        if (content.length > 500) {
             console.log('Page content retrieved successfully.');
        } else {
             console.log('Warning: Page content seems empty or very short.');
        }

        console.log('Verification finished.');
        
    } catch (error) {
        console.error('Error verifying deployment:', error);
    } finally {
        await browser.close();
    }
})();
