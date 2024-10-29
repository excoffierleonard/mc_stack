// Constants for configuration
const CONFIG = {
    AUTO_HIDE_DELAY: 10000,
    ENDPOINTS: {
        CREATE: '/api/v1/create',
        LIST: '/api/v1/list',
        STACK: (id) => `/api/v1/${id}`,
    }
};

// Button state management
const setButtonState = (button, isLoading) => {
    if (!button) return;
    const spinner = button.querySelector('.spinner');
    const textSpan = button.querySelector('.btn-text');
    
    button.disabled = isLoading;
    button.classList.toggle('opacity-75', isLoading);
    button.classList.toggle('cursor-not-allowed', isLoading);
    spinner.classList.toggle('hidden', !isLoading);
    textSpan.classList.toggle('opacity-0', isLoading);
};

// API request handler
async function executeRequest(url, method, buttonElement) {
    if (!buttonElement) return;
    
    const outputElement = document.getElementById("output");
    const errorElement = document.getElementById("error");
    
    [outputElement, errorElement].forEach(el => el.classList.add('hidden'));
    setButtonState(buttonElement, true);

    try {
        const response = await fetch(url, { method });
        const data = await response.text();
        const element = response.ok ? outputElement : errorElement;
        element.innerText = data;
        element.classList.remove('hidden');
        setTimeout(() => element.classList.add('hidden'), CONFIG.AUTO_HIDE_DELAY);
    } finally {
        setButtonState(buttonElement, false);
    }
}

// Stack operations
function createStack() {
    executeRequest(CONFIG.ENDPOINTS.CREATE, "POST", 
        document.querySelector('button[onclick="createStack()"]'));
}

function handleStackOperation(operation, method) {
    const stackId = document.getElementById("stackId").value;
    if (!stackId || isNaN(stackId) || parseInt(stackId) <= 0) return;
    
    executeRequest(CONFIG.ENDPOINTS.STACK(parseInt(stackId)), method,
        document.querySelector(`button[onclick="${operation}()"]`));
}

const deleteStack = () => handleStackOperation('deleteStack', 'DELETE');
const startStack = () => handleStackOperation('startStack', 'PUT');
const stopStack = () => handleStackOperation('stopStack', 'POST');
const listStacks = () => executeRequest(CONFIG.ENDPOINTS.LIST, "GET",
    document.querySelector('button[onclick="listStacks()"]'));
