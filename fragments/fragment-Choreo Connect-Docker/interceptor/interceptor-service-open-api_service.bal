// Copyright (c) 2021, WSO2 Inc. (http://www.wso2.org) All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
import ballerina/http;

listener http:Listener ep0 = new (3000);

service /api/v1 on ep0 {
    resource function post 'handle\-request(@http:Payload RequestHandlerRequestBody payload) returns OkRequestHandlerResponseBody {
        return {body: requestHandler(payload)};
    }
    resource function post 'handle\-response(@http:Payload ResponseHandlerRequestBody payload) returns OkResponseHandlerResponseBody {
        return {body: responseHandler(payload)};
    }
}

function requestHandler(RequestHandlerRequestBody payload) returns RequestHandlerResponseBody {
    Headers headerToBeAdded = {
        "header1": "value1",
        "header2": "value2"
    };

    RequestHandlerResponseBody response = {
        headersToAdd: headerToBeAdded

    };

    return response;

}

function responseHandler(ResponseHandlerRequestBody payload) returns ResponseHandlerResponseBody {
    int statusCode = payload.responseCode; // get backend HTTP status code

    if statusCode == 200 {
        return { // build the response of the interceptor service
            responseCode: 201
        };
    }

    return {}; // if status code is not 200, do not do any changes to backend response
}
