import { authAtom } from "@/state";
import { useAtom, useSetAtom } from "jotai";
import { useState } from "react";
import { Navigate, useNavigate } from "react-router-dom";

export function Login() {
  const auth = localStorage.getItem("auth");
  const setAuth = useSetAtom(authAtom);
  const navigate = useNavigate();

  const [password, setPassword] = useState("");

  if (auth) {
    return <Navigate to="/" replace />;
  }

  return (
    <form>
      <div className="mb-6">
        <label
          htmlFor="password"
          className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
        >
          Your password
        </label>
        <input
          type="password"
          id="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
          required
        />
      </div>
      <button
        type="button"
        onClick={() => {
          setAuth(password);
          navigate("/");
        }}
        className="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
      >
        Submit
      </button>
    </form>
  );
}
