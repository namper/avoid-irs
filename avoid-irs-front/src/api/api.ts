import axios, { AxiosResponse } from "axios";

const API_ENDPOINT = "http://127.0.0.1:8000/api/";

const API_MONTHLY_TAX = API_ENDPOINT + "monthly_tax";

export type IncomeIn = {
  date: Date;
  currency: string;
  amount: number;
};

export type IncomeOut = {
    value: number
};

type PromiseRes<T> = Promise<AxiosResponse<T>>

export const getExchangedCurrency = async (data: IncomeIn): PromiseRes<IncomeOut> => {
  return axios.post(API_MONTHLY_TAX, data);
};
