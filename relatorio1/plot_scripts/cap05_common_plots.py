import matplotlib.pyplot as plt
import numpy as np
from matplotlib import cm


def plot_potential_2d_colormap(potencial: np.ndarray,cmap:str) -> (tuple[plt.Figure,plt.Axes]):
    fig,ax=plt.subplots()
    im = ax.imshow(potencial, cmap=cmap, origin="lower")
    fig.colorbar(im, ax=ax)
    fig.tight_layout()
    return (fig,ax)

def plot_wireframe(potencial: np.ndarray, cmap:str,step: int = 10 ) -> (tuple[plt.Figure,plt.Axes]):    
    ny, nx = potencial.shape
    x = np.arange(nx)
    y = np.arange(ny)
    X, Y = np.meshgrid(x, y)
    fig = plt.figure()
    ax = fig.add_subplot(111, projection="3d")
    norm = plt.Normalize(potencial.min(), potencial.max())
    colors = cm.get_cmap(cmap)(norm(potencial))
    
    rstride = max(1, nx // 100)
    cstride = max(1, ny // 100)
    ax.plot_surface(X, Y, potencial, rstride=rstride, cstride=cstride, facecolors=colors, shade=False)
    
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    ax.set_zlabel("V")
    ax.view_init(elev=30, azim=-60)
    fig.tight_layout()
    return (fig,ax)