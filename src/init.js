const __console_log__ = console.log;
const __console_error__ = console.error;
const __console_warn__ = console.warn;
const __console_info__ = console.info;
const __console_debug__ = console.debug;
const __console_trace__ = console.trace;

function __post_message__(level, message) {
    const request = { type: 'console', parameters: { level: level, message: message } };
    window.rpc.postMessage(JSON.stringify(request));
}

console.log = function (...args) {
    __post_message__('log', args.join(' '));
    __console_log__.apply(console, args);
};

console.error = function (...args) {
    __post_message__('error', args.join(' '));
    __console_error__.apply(console, args);
};

console.warn = function (...args) {
    __post_message__('warn', args.join(' '));
    __console_warn__.apply(console, args);
};

console.info = function (...args) {
    __post_message__('info', args.join(' '));
    __console_info__.apply(console, args);
};

console.debug = function (...args) {
    __post_message__('debug', args.join(' '));
    __console_debug__.apply(console, args);
};

console.trace = function (...args) {
    __post_message__('trace', args.join(' '));
    __console_trace__.apply(console, args);
};

window.rpc = {
    transactions: new Map(),
    nextId: 0,

    postMessage: function (message) {
        return new Promise((resolve, reject) => {
            const messageObj = typeof message === 'string' ? JSON.parse(message) : message;
            const transactionId = this.nextId++;

            if (typeof messageObj.parameters === "undefined") {
                messageObj.parameters = {};
            }
            messageObj.transactionId = transactionId;
            this.transactions.set(transactionId, { resolve, reject });

            setTimeout(() => {
                if (this.transactions.has(transactionId)) {
                    this.transactions.delete(transactionId);
                    reject(new Error('RPC response timeout'));
                }
            }, 10000);

            if (window.ipc) {
                window.ipc.postMessage(JSON.stringify(messageObj));
            } else {
                reject(new Error('IPC not available'));
            }
        });
    },

    handleResponse: function (response) {
        if (response.transactionId !== undefined) {
            const callback = this.transactions.get(response.transactionId);
            if (callback) {
                this.transactions.delete(response.transactionId);
                callback.resolve(response);
            }
        }
    },

    handleError: function (error) {
        if (response.transactionId !== undefined) {
            const callback = this.transactions.get(response.transactionId);
            if (callback) {
                this.transactions.delete(response.transactionId);
                callback.reject(error);
            }
        }
    }
};
