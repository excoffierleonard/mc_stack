// Button state management
const buttonStates = new Map();

// Helper function to manage button state
function setButtonState(button, isLoading) {
    const spinner = button.querySelector('.spinner');
    const textSpan = button.querySelector('.btn-text');
    
    if (isLoading) {
        button.disabled = true;
        button.classList.add('opacity-75', 'cursor-not-allowed');
        spinner.classList.remove('hidden');
        textSpan.classList.add('opacity-0');
    } else {
        button.disabled = false;
        button.classList.remove('opacity-75', 'cursor-not-allowed');
        spinner.classList.add('hidden');
        textSpan.classList.remove('opacity-0');
    }
}

// Helper function to handle button click with loading state
async function handleButtonClick(buttonElement, action) {
    if (buttonStates.get(buttonElement)) return; // Prevent spam clicking
    
    buttonStates.set(buttonElement, true);
    setButtonState(buttonElement, true);
    
    try {
        await action();
    } finally {
        buttonStates.set(buttonElement, false);
        setButtonState(buttonElement, false);
    }
}

// API Functions
async function createStack() {
    const button = document.querySelector('button[onclick="createStack()"]');
    await handleButtonClick(button, async () => {
        await executeRequest("/api/v1/create", "POST");
    });
}

async function deleteStack() {
    const stackId = document.getElementById("stackId").value;
    if (!stackId || isNaN(stackId)) {
        showError("Please enter a valid Stack ID");
        return;
    }
    
    const button = document.querySelector('button[onclick="deleteStack()"]');
    await handleButtonClick(button, async () => {
        await executeRequest(`/api/v1/${parseInt(stackId)}`, "DELETE");
    });
}

async function startStack() {
    const stackId = document.getElementById("stackId").value;
    if (!stackId || isNaN(stackId)) {
        showError("Please enter a valid Stack ID");
        return;
    }
    
    const button = document.querySelector('button[onclick="startStack()"]');
    await handleButtonClick(button, async () => {
        await executeRequest(`/api/v1/${parseInt(stackId)}`, "PUT");
    });
}

async function stopStack() {
    const stackId = document.getElementById("stackId").value;
    if (!stackId || isNaN(stackId)) {
        showError("Please enter a valid Stack ID");
        return;
    }
    
    const button = document.querySelector('button[onclick="stopStack()"]');
    await handleButtonClick(button, async () => {
        await executeRequest(`/api/v1/${parseInt(stackId)}`, "POST");
    });
}

async function listStacks() {
    const button = document.querySelector('button[onclick="listStacks()"]');
    await handleButtonClick(button, async () => {
        await executeRequest("/api/v1/list", "GET");
    });
}

// Helper Functions
async function executeRequest(url, method) {
    const outputElement = document.getElementById("output");
    const errorElement = document.getElementById("error");
    
    // Reset display
    outputElement.classList.add('hidden');
    errorElement.classList.add('hidden');

    try {
        const response = await fetch(url, { 
            method,
            headers: {
                'Content-Type': 'application/json'
            }
        });
        const data = await response.json();

        if (response.ok) {
            showSuccess(data.message);
        } else {
            showError(data.message || 'An error occurred');
        }
    } catch (error) {
        showError('Failed to communicate with the server');
        console.error('Request error:', error);
    }
}

function showSuccess(message) {
    const outputElement = document.getElementById("output");
    outputElement.innerText = message;
    outputElement.classList.remove('hidden');
    
    // Auto-hide success message after 10 seconds
    setTimeout(() => {
        outputElement.classList.add('hidden');
    }, 10000);
}

function showError(message) {
    const errorElement = document.getElementById("error");
    errorElement.innerText = `Error: ${message}`;
    errorElement.classList.remove('hidden');
    
    // Auto-hide error message after 10 seconds
    setTimeout(() => {
        errorElement.classList.add('hidden');
    }, 10000);
}

// Event Listeners
document.addEventListener('DOMContentLoaded', () => {
    // Add input validation
    const stackIdInput = document.getElementById('stackId');
    stackIdInput.addEventListener('input', (e) => {
        if (e.target.value) {
            e.target.classList.remove('input-error');
        }
    });

    // Add keyboard shortcuts
    document.addEventListener('keydown', (e) => {
        if (e.ctrlKey && e.key === 'l') {
            e.preventDefault();
            listStacks();
        }
    });
});
