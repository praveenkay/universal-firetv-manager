import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { App, launchers } from "../src/App";

describe("safe setup workflow", () => {
  it("offers Projectivy and open-source alternatives with provenance", () => {
    expect(launchers.some(item => item.id === "projectivy" && item.user_supplied)).toBe(true);
    expect(launchers.some(item => item.id === "dumbtv-home" && item.open_source)).toBe(true);
  });

  it("requires a numeric IP before compatibility", () => {
    render(<App />);
    fireEvent.click(screen.getByRole("button", { name: "Continue" }));
    const next = screen.getByRole("button", { name: "Continue" });
    fireEvent.change(screen.getByLabelText("Fire TV IP address"), { target: { value: "firetv.local" } });
    expect(next).toBeDisabled();
    expect(screen.getByRole("alert")).toHaveTextContent("numeric IPv4");
  });

  it("explains mandatory backup limitations", () => {
    render(<App />);
    for (let index = 0; index < 4; index += 1) {
      if (index === 1) fireEvent.change(screen.getByLabelText("Fire TV IP address"), { target: { value: "192.168.1.50" } });
      fireEvent.click(screen.getByRole("button", { name: "Continue" }));
    }
    expect(screen.getByRole("heading", { name: /mandatory baseline backup/i })).toBeInTheDocument();
    expect(screen.getByText(/cannot copy private app logins/i)).toBeInTheDocument();
  });
});
