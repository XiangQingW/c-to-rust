#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef unsigned char u8;
typedef unsigned int u32;

enum HttpMethod {
		 GET,
		 POST
};

struct HttpHeader {
  char* name;
  char* value;
};

struct HttpRequest {
  enum HttpMethod method;
  char* url;
  int timeout;

  u32 body_len;
  u8* body;

  u32 header_len;
  struct HttpHeader* headers;
};

struct HttpResponse {
  int status_code;
  char* unknown_err;

  u32 body_len;
  u8* body;

  u32 header_len;
  struct HttpHeader* headers;
};

extern struct HttpResponse* send_http(struct HttpRequest* req);
extern void free_http_response(struct HttpResponse* resp);

int main() {
  struct HttpRequest *req = malloc(sizeof(struct HttpRequest));
  req->timeout = 20;
  req->url = "https://api.zjurl.cn";

  enum HttpMethod method = POST;
  req->method = method;

  req->body_len = 3;
  req->body = malloc(req->body_len * sizeof(u8));
  req->body[0] = 0;
  req->body[1] = 1;
  req->body[2] = 2;

  req->header_len = 2;
  req->headers = malloc(req->header_len * sizeof(struct HttpHeader));

  struct HttpHeader header1;
  header1.name = "name1";
  header1.value = "value1";
  req->headers[0] = header1;

  struct HttpHeader header2;
  header2.name = "name2";
  header2.value = "value2";
  req->headers[1] = header2;

  struct HttpResponse* resp = send_http(req);

  printf("status_code= %d unknown_err= %s \n", resp->status_code, resp->unknown_err);

  printf("http body: ");
  for (int i = 0; i < resp->body_len; i ++) {
    printf("%d ", resp->body[i]);
  }
  printf("\n");

  printf("http headers: \n");
  for (int i = 0; i < resp->header_len; i ++) {
    printf("name= %s value= %s\n", resp->headers[i].name, resp->headers[1].value);
  }

  free_http_response(resp);
  free(req);

  return 0;
}
