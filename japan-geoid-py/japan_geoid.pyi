from typing import Self

from numpy import ndarray

class GsiGeoid:
    """GSIGEO2011 geoid model for Japan."""

    @classmethod
    def from_ascii(cls, content: str) -> Self:
        """Load the geoid model from the original ascii format."""
        return japan_geoid.GsiGeoid.from_ascii(content)

    @classmethod
    def from_binary(cls, content: bytes) -> Self:
        """Load the geoid model from the efficient binary format."""

    def to_binary(self) -> bytes:
        """Serialize the geoid model in the efficient binary format."""

    def get_height(self, lng: float, lat: float) -> float:
        """Get the geoid height at a specified point."""

    def get_heights(self, lng: ndarray, lat: ndarray) -> float:
        """Get the geoid height at each specified point."""
