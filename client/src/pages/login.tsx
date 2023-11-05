import { tokenAtom } from "./_state";
import { Button, Card, Input, Title } from "@mantine/core";
import { useAtom } from "jotai";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

export default function Login() {
  const [token, setToken] = useAtom(tokenAtom);
  const navigate = useNavigate();

  const [password, setPassword] = useState("");

  useEffect(() => {
    if (token) {
      navigate("/", { replace: true });
    }
  }, [token]);

  return (
    <div className="w-full flex flex-col items-center px-3">
      <Card
        shadow="sm"
        padding="lg"
        radius="md"
        withBorder
        className="w-full md:w-[40rem] mt-4"
      >
        <form
          className="flex flex-col items-center"
          onSubmit={(e) => e.preventDefault()}
        >
          <Title className="py-5">Login</Title>
          <div className="mb-6 w-full">
            <label
              htmlFor="password"
              className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
            >
              Enter your password
            </label>
            <Input
              type="password"
              id="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className=""
              required
            />
          </div>
          <Button
            type="submit"
            fullWidth
            onClick={() => {
              setToken(password);
            }}
            className=""
          >
            Submit
          </Button>
        </form>
      </Card>
    </div>
  );
}
