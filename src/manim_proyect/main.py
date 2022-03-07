from manim import *

class ManimProyect(Scene):
    def construct(self):
        self: Scene

        text = Text("Welcome to manim_proyect")

        self.play(Write(text))
        self.wait(0.2)
        self.play(FadeOut(text))