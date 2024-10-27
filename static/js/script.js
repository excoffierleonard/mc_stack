// Constants for configuration
const CONFIG = {
    AUTO_HIDE_DELAY: 10000,
    ENDPOINTS: {
        CREATE: '/api/v1/create',
        LIST: '/api/v1/list',
        STACK: (id) => `/api/v1/${id}`,
    }
};

// Button state management with WeakMap for better memory management
const buttonStates = new WeakMap();

// Helper function to manage button state
function setButtonState(button, isLoading) {
    const spinner = button.querySelector('.spinner');
    const textSpan = button.querySelector('.btn-text');
    
    const states = {
        loading: ['opacity-75', 'cursor-not-allowed'],
        text: ['opacity-0']
    };
    
    button.disabled = isLoading;
    button.classList.toggle(states.loading[0], isLoading);
    button.classList.toggle(states.loading[1], isLoading);
    spinner.classList.toggle('hidden', !isLoading);
    textSpan.classList.toggle(states.text[0], isLoading);
}

// Enhanced error handling with type checking
class APIError extends Error {
    constructor(message, status) {
        super(message);
        this.status = status;
        this.name = 'APIError';
    }
}

// Helper function to handle button click with loading state
async function handleButtonClick(buttonElement, action) {
    if (!buttonElement || buttonStates.get(buttonElement)) return;
    
    buttonStates.set(buttonElement, true);
    setButtonState(buttonElement, true);
    
    try {
        await action();
    } catch (error) {
        // Log errors with additional context
        console.error('Action failed:', {
            action: action.name,
            error: error.message,
            status: error.status,
            timestamp: new Date().toISOString()
        });
        throw error;
    } finally {
        buttonStates.set(buttonElement, false);
        setButtonState(buttonElement, false);
    }
}

// API Functions with input validation
async function createStack() {
    const button = document.querySelector('button[onclick="createStack()"]');
    await handleButtonClick(button, () => 
        executeRequest(CONFIG.ENDPOINTS.CREATE, "POST")
    );
}

async function handleStackOperation(operation, method) {
    const stackId = document.getElementById("stackId").value;
    if (!validateStackId(stackId)) {
        throw new APIError("Please enter a valid Stack ID", 400);
    }
    
    const button = document.querySelector(`button[onclick="${operation}()"]`);
    await handleButtonClick(button, () => 
        executeRequest(CONFIG.ENDPOINTS.STACK(parseInt(stackId)), method)
    );
}

// Simplified stack operations using the handler
const deleteStack = () => handleStackOperation('deleteStack', 'DELETE');
const startStack = () => handleStackOperation('startStack', 'PUT');
const stopStack = () => handleStackOperation('stopStack', 'POST');
const listStacks = () => handleButtonClick(
    document.querySelector('button[onclick="listStacks()"]'),
    () => executeRequest(CONFIG.ENDPOINTS.LIST, "GET")
);

// Enhanced request execution with better error handling
async function executeRequest(url, method) {
    const outputElement = document.getElementById("output");
    const errorElement = document.getElementById("error");
    
    // Reset display
    [outputElement, errorElement].forEach(el => el.classList.add('hidden'));

    try {
        const response = await fetch(url, { 
            method,
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            }
        });
        
        const data = await response.json();

        if (!response.ok) {
            throw new APIError(data.message || 'Server error', response.status);
        }

        showSuccess(data.message);
    } catch (error) {
        if (error instanceof APIError) {
            showError(error.message);
        } else {
            showError('Failed to communicate with the server');
        }
        throw error; // Re-throw for higher-level handling if needed
    }
}

// Utility function for stack ID validation
function validateStackId(stackId) {
    return stackId && !isNaN(stackId) && parseInt(stackId) > 0;
}

// Message display functions with automatic cleanup
function showMessage(element, message, prefix = '') {
    if (!element) return;
    
    clearTimeout(element.hideTimeout);
    element.innerText = prefix ? `${prefix}: ${message}` : message;
    element.classList.remove('hidden');
    
    element.hideTimeout = setTimeout(() => {
        element.classList.add('hidden');
    }, CONFIG.AUTO_HIDE_DELAY);
}

const showSuccess = message => showMessage(document.getElementById("output"), message);
const showError = message => showMessage(document.getElementById("error"), message, 'Error');

// Event Listeners with cleanup
document.addEventListener('DOMContentLoaded', () => {
    const stackIdInput = document.getElementById('stackId');
    if (!stackIdInput) return;

    // Debounced input validation
    let validationTimeout;
    stackIdInput.addEventListener('input', (e) => {
        clearTimeout(validationTimeout);
        validationTimeout = setTimeout(() => {
            if (e.target.value) {
                e.target.classList.remove('input-error');
            }
        }, 300);
    });
});
