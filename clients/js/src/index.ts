import axios, {type AxiosInstance, type AxiosRequestConfig, type AxiosResponse } from 'axios';

export default class ApiClient {
    private client: AxiosInstance;
    baseURL: string;

    constructor(baseURL: string) {
        this.client = axios.create({
            baseURL
        });
        this.baseURL = baseURL;

        console.log(baseURL)
    }

    public static createInstance(baseURL: string): ApiClient {
        return new ApiClient(baseURL);
    }

    public async get<T>(url: string, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {
        return this.client.get<T>(url, config);
    }

    public async post<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {
        return this.client.post<T>(url, data, config);
    }

    public async put<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {
        return this.client.put<T>(url, data, config);
    }

    public async patch<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {
        return this.client.patch<T>(url, data, config);
    }

    public async delete<T>(url: string, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {
        return this.client.delete<T>(url, config);
    }

    public async head<T>(url: string, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {
        return this.client.head<T>(url, config);
    }

    public async options<T>(url: string, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {
        return this.client.options<T>(url, config);
    }

    public async request<T>(config: AxiosRequestConfig): Promise<AxiosResponse<T>> {
        return this.client.request<T>(config);
    }
}

import { type Config, type ConfigRequest, type ConfigResponse } from './models';
import { fetchConfig } from './config';

export { type Config, type ConfigRequest, type ConfigResponse, fetchConfig };
export const apiClient = ApiClient.createInstance('http://localhost:6968');
