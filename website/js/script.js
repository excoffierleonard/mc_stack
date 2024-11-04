// Constants for configuration
const CONFIG = {
    AUTO_HIDE_DELAY: 5000,
    ENDPOINTS: {
        CREATE: '/api/v1/stacks',
        LIST: '/api/v1/stacks',
        STACK: (id) => `/api/v1/stacks/${id}`,
        STACK_STATUS: (id) => `/api/v1/stacks/${id}/status`,
    },
    STATUS_CLASSES: {
        success: 'bg-green-50 text-green-700',
        error: 'bg-red-50 text-red-700'
    }
};

// Show status message
const showStatus = (message, type = 'success') => {
    const statusElement = document.getElementById('statusMessage');
    statusElement.textContent = message;
    statusElement.className = `rounded-lg p-4 ${CONFIG.STATUS_CLASSES[type]}`;
    statusElement.classList.remove('hidden');
    setTimeout(() => statusElement.classList.add('hidden'), CONFIG.AUTO_HIDE_DELAY);
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

// Update the server card creation (wan_ip now comes from stack object)
const createServerCard = (stack) => {
    const mcStatus = stack.services.minecraft_server;
    const sftpStatus = stack.services.sftp_server;
    
    return `
        <div class="bg-gray-50 rounded-lg p-4 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
            <div class="space-y-2">
                <div class="font-semibold text-gray-800">Stack ${stack.stack_id} <span class="text-gray-500 text-sm ml-2">IP: ${stack.wan_ip || 'Not available'}</span></div>
                <div class="text-sm text-gray-600">
                    <div>Minecraft Server: ${mcStatus.status} ${mcStatus.port ? `(Port: ${mcStatus.port})` : ''}</div>
                    <div>SFTP Server: ${sftpStatus.status} ${sftpStatus.port ? `(Port: ${sftpStatus.port})` : ''}</div>
                </div>
            </div>
            <div class="flex gap-2 w-full sm:w-auto">
                ${mcStatus.status === 'stopped' ? `
                    <button
                        onclick="updateStackStatus(${stack.stack_id}, 'running')"
                        class="btn-action bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded-lg transition duration-200 flex-1 sm:flex-none"
                    >
                        <span class="btn-text">Start</span>
                        <div class="spinner hidden"></div>
                    </button>
                ` : `
                    <button
                        onclick="updateStackStatus(${stack.stack_id}, 'stopped')"
                        class="btn-action bg-yellow-500 hover:bg-yellow-600 text-white font-semibold py-2 px-4 rounded-lg transition duration-200 flex-1 sm:flex-none"
                    >
                        <span class="btn-text">Stop</span>
                        <div class="spinner hidden"></div>
                    </button>
                `}
                <button
                    onclick="deleteStack(${stack.stack_id})"
                    class="btn-action bg-red-500 hover:bg-red-600 text-white font-semibold py-2 px-4 rounded-lg transition duration-200 flex-1 sm:flex-none"
                >
                    <span class="btn-text">Delete</span>
                    <div class="spinner hidden"></div>
                </button>
            </div>
        </div>
    `;
};

// Update server list function to handle new response structure
const updateServerList = (stacks) => {
    const serverList = document.getElementById('serverList');
    if (!stacks || stacks.length === 0) {
        serverList.innerHTML = '<div class="text-gray-500 text-center py-8">No servers available</div>';
        return;
    }
    serverList.innerHTML = stacks.map(stack => createServerCard(stack, stack.wan_ip)).join('');
};

// Updated API request handler to handle new status codes and response structures
async function executeRequest(url, method, buttonElement, body = null) {
    setButtonState(buttonElement, true);

    try {
        const options = {
            method,
            headers: {}
        };

        if (body) {
            options.headers['Content-Type'] = 'application/json';
            options.body = JSON.stringify(body);
        }

        const response = await fetch(url, options);
        
        // Handle 204 No Content responses
        if (response.status === 204) {
            showStatus('Operation completed successfully', 'success');
            if (method !== 'GET') {
                refreshServerList();
            }
            return null;
        }

        // For responses with content
        const isJson = response.headers.get('content-type')?.includes('application/json');
        const data = isJson ? await response.json() : null;

        if (response.ok) {
            // For successful GET with content or POST with created resource
            if (method === 'POST') {
                showStatus(`Stack ${data.stack_id} created successfully`, 'success');
            }
            if (method !== 'GET') {
                refreshServerList();
            }
            return data;
        } else {
            // Error cases with message
            const errorMessage = data?.message || 'An error occurred';
            showStatus(errorMessage, 'error');
        }
    } catch (error) {
        showStatus('An error occurred while processing your request', 'error');
    } finally {
        setButtonState(buttonElement, false);
    }
}

// Stack operations
async function createStack() {
    await executeRequest(
        CONFIG.ENDPOINTS.CREATE,
        'POST',
        document.querySelector('button[onclick="createStack()"]')
    );
}

// Update refresh server list to handle new response structure
async function refreshServerList() {
    const data = await executeRequest(
        CONFIG.ENDPOINTS.LIST,
        'GET',
        document.querySelector('button[onclick="refreshServerList()"]')
    );
    if (data) updateServerList(data);
}

async function updateStackStatus(id, status) {
    await executeRequest(
        CONFIG.ENDPOINTS.STACK_STATUS(id),
        'PATCH',
        document.querySelector(`button[onclick="updateStackStatus(${id}, '${status}')"]`),
        { status }
    );
}

async function deleteStack(id) {
    if (!confirm('Are you sure you want to delete this stack? This action cannot be undone.')) return;
    
    await executeRequest(
        CONFIG.ENDPOINTS.STACK(id),
        'DELETE',
        document.querySelector(`button[onclick="deleteStack(${id})"]`)
    );
}

// Initial load
document.addEventListener('DOMContentLoaded', refreshServerList);