using UnityEngine;

public class ImageRenderer : MonoBehaviour
{

    ThumbnailSetup objprefabs;
    private static ImageRenderer instance;

    private Camera myCamera;
    private bool takeScreenshotOnNextFrame;

    private void Awake()
    {
        instance = this;
        myCamera = gameObject.GetComponent<Camera>();
        objprefabs = GetComponent<ThumbnailSetup>();
    }

    private void OnPostRender()
    {
        if (takeScreenshotOnNextFrame)
        {
            takeScreenshotOnNextFrame = false;
            RenderTexture renderTexture = myCamera.targetTexture;

            Texture2D renderResult = new Texture2D(renderTexture.width, renderTexture.height, TextureFormat.ARGB32, false);
            Rect rect = new Rect(0, 0, renderTexture.width, renderTexture.height);
            renderResult.ReadPixels(rect, 0, 0);

            byte[] byteArray = renderResult.EncodeToPNG();

            if (objprefabs.selectCustomFolder)
            {
                System.IO.File.WriteAllBytes(objprefabs.folderPath + "/" + objprefabs.PrefabName() + ".png", byteArray);
            }
            else
            {
                System.IO.File.WriteAllBytes(Application.dataPath + "/" + objprefabs.PrefabName() + ".png", byteArray);
            }

            RenderTexture.ReleaseTemporary(renderTexture);
            myCamera.targetTexture = null;
        }
    }

    private void TakeScreenshot(int width, int height)
    {
        myCamera.targetTexture = RenderTexture.GetTemporary(width, height, 16);
        takeScreenshotOnNextFrame = true;
    }

    public static void TakeScreenshot_Static(int width, int height)
    {
        instance.TakeScreenshot(width, height);
    }
}
