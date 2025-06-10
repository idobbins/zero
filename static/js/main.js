// Main JavaScript file for Zero App
// Add your global JavaScript functionality here

document.addEventListener('DOMContentLoaded', function() {
    console.log('Zero App initialized');
    
    // Example: Add smooth scrolling to anchor links
    const anchorLinks = document.querySelectorAll('a[href^="#"]');
    anchorLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth'
                });
            }
        });
    });
    
    // Example: Add click handlers for buttons with data attributes
    const actionButtons = document.querySelectorAll('[data-action]');
    actionButtons.forEach(button => {
        button.addEventListener('click', function() {
            const action = this.getAttribute('data-action');
            console.log(`Action triggered: ${action}`);
            // Handle different actions here
        });
    });
});
