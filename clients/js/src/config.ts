import { apiClient } from "./index";
import { type Config, type ConfigRequest, type ConfigResponse } from "./models";

/**
 * Fetches configuration data from the API based on the provided request parameters.
 *
 * @param {ConfigRequest} data - The request parameters to fetch the configuration data.
 * @return {Config | null} The fetched configuration data, or null if the request fails.
 */
export async function fetchConfig(data: ConfigRequest): Promise<Config | null> {
    try {
        const res = await apiClient.get<ConfigResponse>("/api/config", { data: data, baseURL: apiClient.baseURL });

        // Handle different status codes
        if (res.status >= 200 && res.status < 300) {
            // Success
            return res.data.config;
        } else if (res.status >= 400 && res.status < 500) {
            // Client error
            console.warn(`Client error: ${res.status} - ${res.statusText}`);
            return null;  // or you could return a default config if applicable
        } else if (res.status >= 500) {
            // Server error
            console.error(`Server error: ${res.status} - ${res.statusText}`);
            return null;  // or handle it according to your app's needs
        } else {
            // Unexpected status code
            console.warn(`Unexpected status code: ${res.status} - ${res.statusText}`);
            return null;
        }
    } catch (error) {
        console.error("Network or other unexpected error:", error);
        return null;  // or throw the error if you want to handle it higher up
    }
}