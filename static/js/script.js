// API Functions
async function createStack() {
    await executeRequest("/api/v1/create", "POST");
}

async function deleteStack() {
    const stackId = document.getElementById("stackId").value;
    if (!stackId) {
        showError("Please enter a Stack ID");
        return;
    }
    await executeRequest(`/api/v1/${stackId}`, "DELETE");
}

async function startStack() {
    const stackId = document.getElementById("stackId").value;
    if (!stackId) {
        showError("Please enter a Stack ID");
        return;
    }
    await executeRequest(`/api/v1/${stackId}`, "PUT");
}

async function stopStack() {
    const stackId = document.getElementById("stackId").value;
    if (!stackId) {
        showError("Please enter a Stack ID");
        return;
    }
    await executeRequest(`/api/v1/${stackId}`, "POST");
}

async function listStacks() {
    await executeRequest("/api/v1/list", "GET");
}

// Helper Functions
async function executeRequest(url, method) {
    const outputElement = document.getElementById("output");
    const errorElement = document.getElementById("error");
    
    // Reset display
    outputElement.classList.add('hidden');
    errorElement.classList.add('hidden');
    
    // Show loading state
    const spinner = document.getElementById('createSpinner');
    spinner.classList.remove('hidden');

    try {
        const response = await fetch(url, { method });
        const data = await response.json();

        if (response.ok) {
            showSuccess(data.message);
        } else {
            showError(data.message || 'An error occurred');
        }
    } catch (error) {
        showError('Failed to communicate with the server');
    } finally {
        spinner.classList.add('hidden');
    }
}

function showSuccess(message) {
    const outputElement = document.getElementById("output");
    outputElement.innerText = message;
    outputElement.classList.remove('hidden');
}

function showError(message) {
    const errorElement = document.getElementById("error");
    errorElement.innerText = `Error: ${message}`;
    errorElement.classList.remove('hidden');
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
        if (e.ctrlKey || e.metaKey) {
            switch(e.key) {
                case 'l':
                    e.preventDefault();
                    listStacks();
                    break;
                case 'c':
                    e.preventDefault();
                    createStack();
                    break;
            }
        }
    });
});
