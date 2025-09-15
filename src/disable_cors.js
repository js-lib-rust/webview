// Override XMLHttpRequest
const originalOpen = XMLHttpRequest.prototype.open;
XMLHttpRequest.prototype.open = function () {
    originalOpen.apply(this, arguments);
    this.withCredentials = false; // Disable credentials
};

// Override fetch
const originalFetch = window.fetch;
window.fetch = function () {
    return originalFetch.apply(this, arguments).then(response => {
        // Modify response headers to allow CORS
        const modifiedResponse = response.clone();
        Object.defineProperty(modifiedResponse, 'headers', {
            value: new Headers({
                'Access-Control-Allow-Origin': '*',
                'Access-Control-Allow-Methods': '*',
                'Access-Control-Allow-Headers': '*'
            })
        });
        return modifiedResponse;
    });
};
