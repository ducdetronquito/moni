import app/router
import gleeunit
import gleeunit/should
import wisp/testing

pub fn main() {
  gleeunit.main()
}

pub fn request_is_handled_test() {
  let request = testing.get("/", [])
  let response = router.handle_request(request)

  response.status
  |> should.equal(200)
}
