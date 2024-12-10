from py_raytrace import tracer as t


camera = t.Camera(
    defocus_angle=0.8,
    resolution=[360, 270],
    yfov=75,
    viewport_distance=4.0,
    pos=[-2.5, 3, -1],
    up=[0, 1, 0],
    lookat=[1, 0, -0.2],
)

scene = t.Scene("config/cornell/scene.toml")

r = t.RayTracer(
    camera,
    scene,
    spp=8,
    out_path="pyout.png",
)

r.render()
