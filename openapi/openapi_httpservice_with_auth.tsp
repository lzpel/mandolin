import "@typespec/http";

using TypeSpec.Http;
@service({
  title: "Widget Service",
})
namespace DemoService;

alias MyOAuth2<Scopes extends string[]> = OAuth2Auth<
  [
    {
      type: OAuth2FlowType.implicit;
      authorizationUrl: "https://api.example.com/oauth2/authorize";
      refreshUrl: "https://api.example.com/oauth2/refresh";
    }
  ],
  Scopes
>;

model Widget {
  @visibility("read", "update")
  @path
  id: string;

  weight: int32;
  color: "red" | "blue";
}

@error
model Error {
  code: int32;
  message: string;
}

@route("/widgets")
@tag("Widgets")
interface Widgets {
  @useAuth(BasicAuth)  @get list(): Widget[] | Error;
  @useAuth(BearerAuth) @get read(@path id: string): Widget | Error;
  @useAuth(MyOAuth2<["create"]>) @post create(...Widget): Widget | Error;
  @useAuth(MyOAuth2<["write"]>) @patch update(...Widget): Widget | Error;
  @delete delete(@path id: string): void | Error;
  @route("{id}/analyze") @post analyze(@path id: string): string | Error;
}
