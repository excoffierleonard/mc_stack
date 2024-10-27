async function createStack() {
    const response = await fetch("/api/v1/create", { method: "POST" });
    handleResponse(response);
}
  
async function deleteStack() {
    const stackId = document.getElementById("stackId").value;
    const response = await fetch(`/api/v1/${stackId}`, { method: "DELETE" });
    handleResponse(response);
}
  
async function startStack() {
    const stackId = document.getElementById("stackId").value;
    const response = await fetch(`/api/v1/${stackId}`, { method: "PUT" });
    handleResponse(response);
}

async function stopStack() {
    const stackId = document.getElementById("stackId").value;
    const response = await fetch(`/api/v1/${stackId}`, { method: "POST" });
    handleResponse(response);
}
  
async function listStacks() {
    const response = await fetch("/api/v1/list", { method: "GET" });
    handleResponse(response);
}
  
async function handleResponse(response) {
    const outputElement = document.getElementById("output");
    const errorElement = document.getElementById("error");
    outputElement.innerText = ""; // Clear the output
    errorElement.innerText = ""; // Clear previous errors
    if (response.ok) {
        const data = await response.json();
        outputElement.innerText = data.message;
    } else {
        const errorData = await response.json().catch(() => ({ message: 'Unknown error occurred' }));
        errorElement.innerText = `Error: ${errorData.message}`;
    }
}
