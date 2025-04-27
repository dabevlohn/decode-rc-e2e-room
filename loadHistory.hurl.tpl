POST https://%%server%%/api/v1/login
[FormParams]
user: %%user%%
password: %%password%%
HTTP 200
[Captures]
authToken: jsonpath "$['data']['authToken']"
userId: jsonpath "$['data']['userId']"

POST https://%%server%%/api/v1/method.call/loadHistory
X-User-Id: {{userId}}
X-Auth-Token: {{authToken}}
[FormParams]
message: {\"msg\":\"method\",\"id\":\"10\",\"method\":\"loadHistory\",\"params\":[\"%%roomid%%\",null,50,\"Thu Apr 03 2024 15:43:28 GMT+0300 (Moscow Standard Time)\",false]}
