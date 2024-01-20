from typing import Self

from numpy import ndarray

def load_embedded_gsigeo2011() -> GsiGeoid:
    """Load the embedded GSIGEO2011 Japan geoid model."""

class GsiGeoid:
    """GSIGEO2011 geoid model for Japan."""

    @classmethod
    def from_ascii(cls, content: str) -> Self:
        """Load the geoid model from the original ascii format."""

    @classmethod
    def from_binary(cls, content: bytes) -> Self:
        """Load the geoid model from the efficient binary format."""

    def to_binary(self) -> bytes:
        """Serialize the geoid model in the efficient binary format."""

    def get_height(self, lng: float, lat: float) -> float:
        """Get the geoid height at a specified point."""

    def get_heights(self, lng: ndarray, lat: ndarray) -> float:
        """Get the geoid height at each specified point."""
