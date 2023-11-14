```md
openai-client-cli 0.1.1 by Asher Jingkong Chen
OpenAI API client CLI

Usage: openai-client [OPTIONS] <PATH>

Arguments:
  <PATH>
          The API request path. (part of the URL)
          The program will use regex to extract the matched segment in <PATH>.
          For example, the extracted strings will be the same when <PATH> is either
          `chat/completions`, `/chat/completions` or `https://api.openai.com/v1/chat/completions`.

Options:
  -k, --key-file <KEY_FILE_PATH>
          The file path where the API key is stored.
          The program will attempt the following steps to obtain a valid API key:
           1. Read the file from the provided path <KEY_FILE_PATH>.
           2. Read the environment variable `OPENAI_API_KEY`.
           3. Read the file from the default paths in the following order:
              `openai.env`, `.openai_profile`, `.env`,
              `~/openai.env`, `~/.openai_profile` or `~/.env`.
           4. Exit the program with a non-zero return code.
          
  -m, --method <METHOD>
          The HTTP method used for the API request.
          The program will attempt the following steps to determine a valid HTTP method:
           1. Read the value of argument <METHOD>.
           2. If the `parameter` object is successfully fetched from either
              <PARAM_FILE_PATH> or one of the default paths, set <METHOD> to `POST`.
           3. Otherwise, set <METHOD> to `GET`.
          
  -g, --org-file <ORG_FILE_PATH>
          The file path where the organization ID is stored.
          The program will attempt the following steps to obtain a valid organization ID:
           1. Read the file from the provided path <ORG_FILE_PATH>.
           2. Read the file from provided path of key file <KEY_FILE_PATH>.
           3. Read the environment variable `OPENAI_ORG_KEY`.
           4. Read the file from the default paths in the following order:
              `openai.env`, `.openai_profile`, `.env`,
              `~/openai.env`, `~/.openai_profile` or `~/.env`.
           5. Ignore the field and leave it empty.
          
  -o, --output-file <OUTPUT_FILE_PATH>
          The file path where the API response will be stored.
          The program will attempt the following steps to successfully store the response:
           1. Export the output to the provided file path <OUTPUT_FILE_PATH>.
           2. Export the output to the standard output.
           3. Exit the program with a non-zero return code.
          
  -p, --parameter-file <PARAM_FILE_PATH>
          The file path where the API request parameters (body) are stored in JSON format.
          The program will attempt the following steps to obtain a valid parameter object:
           1. Read the file from the provided path <PARAM_FILE_PATH>.
           2. Read the file from the default paths in the following order:
              `openai.json`, `openai-parameters.json`, `openai_parameters.json`,
              `openai-parameters`, `openai_parameters`, or `openai.config.json`.
           3. Ignore the field and leave it empty
          
  -v, --verbose
          Switch for verbose logging mode. This mode is useful for debugging purposes.
          It is disabled by default.
          
  -h, --help
          Print help
  -V, --version
          Print version
```
