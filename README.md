# Currency Converter
# Currency Converter

## Description

This is a simple currency converter that converts an amount from one currency to another using the Exchange Rate API.

## Environment Variables

- `EXCHANGE_RATE_API_KEY`: The API key for the Exchange Rate API. You can get it from [here](https://www.exchangerate-api.com/dashboard).

## Parameters

- `-a`, `--amount`: The amount to convert.
- `-f`, `--from`: The currency to convert from.
- `-t`, `--to`: The currency to convert to.

## Example

```bash
./currency_converter --amount 200 --from PHP --to USD
200 PHP is 3.46 USD
```

```bash
./currency_converter --amount 20 --from USD
Enter currency to convert into: PHP
20 USD is 1,156.55 PHP
```

```bash
./currency_converter --amount 30
Enter currency to convert from: USD
Enter currency to convert into: PHP
30 USD is 1,734.82 PHP
```

```bash
./currency_converter
Enter amount: 200
Enter currency to convert from: PHP
Enter currency to convert into: USD
100 PHP is 3.46 USD
```







