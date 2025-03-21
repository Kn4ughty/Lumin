from dataclasses import dataclass, field
from typing import List


# https://specifications.freedesktop.org/desktop-entry-spec/latest/
@dataclass
class DesktopApp:
    name: str
    cmd_to_execute: List[str]
    generic_name: str = ""
    keywords: list[str] = field(default_factory=list)
    catagoires: list[str] = field(default_factory=list)
    icon: None = None
    terminal: bool = False
