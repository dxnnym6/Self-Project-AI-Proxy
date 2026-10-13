from fastapi import FastAPI #for server framework
from pydantic import BaseModel #for data validation
import re #for pattern searching

app = FastAPI(title="AI Security Sidecar")

class ScanRequest(BaseModel):
        prompt: str

INJECTION_PATTERNS = [
        r"ignore previous instructions",
        r"system prompt override",
        r"you are now unfiltered",
        r"disregard all prior directives",
        r"jailbreak",
]

@app.post("/inspect")
async def inspect_prompt(req: ScanRequest):
        prompt_low = req.prompt.lower()
        for pattern in INJECTION_PATTERNS:
                if re.search(pattern, prompt_low):
                        print(f"[SECURITY ALERT] Blocked injection pattern: '{pattern}'")
                        return {
                                "safe": False,
                                "reason": f"Detected forbidden pattern: '{pattern}'"
                        }

        return {"safe": True, "reason": "Prompt is safe"}

if __name__ == "__main__":
        import uvicorn
        uvicorn.run(app, host="127.0.0.1", port=8000)