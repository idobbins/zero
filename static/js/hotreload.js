// Hot-reload functionality for Zero App
(function() {
    let eventSource;
    let isPolling = false;
    
    function connectSSE() {
        eventSource = new EventSource('/events');
        
        eventSource.onopen = function() {
            console.log('Hot-reload: Connected to server');
        };
        
        eventSource.onmessage = function(event) {
            console.log('Hot-reload: Received event:', event.data);
            
            if (event.data === 'shutdown') {
                console.log('Hot-reload: Server shutdown detected, starting health polling...');
                eventSource.close();
                startHealthPolling();
            }
        };
        
        eventSource.onerror = function() {
            console.log('Hot-reload: SSE connection error');
            eventSource.close();
            // If we lose connection unexpectedly, start polling
            if (!isPolling) {
                startHealthPolling();
            }
        };
    }
    
    function startHealthPolling() {
        if (isPolling) return;
        isPolling = true;
        
        const pollHealth = async () => {
            try {
                const response = await fetch('/health');
                if (response.ok) {
                    console.log('Hot-reload: Server is back up, refreshing page...');
                    window.location.reload();
                    return;
                }
            } catch (error) {
                // Server still down, continue polling
            }
            
            // Poll again in 50ms
            setTimeout(pollHealth, 50);
        };
        
        pollHealth();
    }
    
    // Start SSE connection when page loads
    connectSSE();
})();
