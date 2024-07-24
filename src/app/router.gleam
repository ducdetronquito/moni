import gleam/string_builder
import wisp.{type Request, type Response}

pub fn default_middleware(
  request: Request,
  handle_request: fn(Request) -> Response,
) -> Response {
  use <- wisp.log_request(request)
  use <- wisp.rescue_crashes()
  handle_request(request)
}

pub fn handle_request(request: Request) -> Response {
  use _request <- default_middleware(request)
  let body = string_builder.from_string("Hello !")
  wisp.html_response(body, 200)
}
