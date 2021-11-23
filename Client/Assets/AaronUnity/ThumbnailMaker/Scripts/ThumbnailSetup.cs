using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ThumbnailSetup : MonoBehaviour
{
    [Header("Background Setting")]
    [SerializeField] GameObject background;
    [SerializeField] Material backgroundMaterial;
    [Tooltip("Set background color if turned on. If turned off there will be no background")]
    [SerializeField] bool enableBackground = false;
    [SerializeField] Color backgroundColor;
    [Space]
    [Space]

    private Camera myCamera;

    [Tooltip("Resolution and aspect ratio of generated Thumbnail. You can make it a square or rectangle")]
    [Header("Thumbnail resolution/size")]
    [Space]
    [Space]
    [SerializeField] int width = 2048;
    [SerializeField] int height = 2048;

    [Tooltip("Select between Orthographic and Perspective mode")]
    [Header("Camera Mode")]
    [Space]
    [Space]
    [SerializeField] bool Orthographic = true;

    [Tooltip("The size of the blank space around the asset being snapshot")]
    [Header("Orthographic mode Thumbnail Frame")]
    [SerializeField] [Range(-1f, 1f)] float orthographicFrameSize = 0.1f;

    [Tooltip("The blank space around the object on the selected axis. IF your asseta are too big or too small" +
        "you can try diferent margins to better acomodate your thumbnails")]
    [Header("Perspective mode Thumbnail frame")]
    [Space]
    [Space]
    [SerializeField] [Range(-1f, 1f)] float perspectiveFrameSize = 0f;


    [Header("SELECT ONLY ONE")]
    [Header("Align the camera to the center of every object")]
    [Space]
    [Space]
    [Space]
    [SerializeField] bool facingX = false;
    [SerializeField] bool facingNegX = false;
    [SerializeField] bool facingY = false;
    [SerializeField] bool facingNegY = false;
    [SerializeField] bool facingZ = false;
    [SerializeField] bool facingNegZ = false;


    [Tooltip("Great if you offset the camera")]
    [Header("Make camera look at the center of Asset")]
    [Space]
    [Space]
    [SerializeField] bool lookAtObject = true;

    [Tooltip("Change camera position from default")]
    [Header("Camera Offset Position")]
    [Space]
    [Space]
    [SerializeField] Vector3 camOffset;
    Vector3 cameraOffset;

    [Tooltip("The rotation in which your asset will be instantiated")]
    [Header("Asset Rotation")]
    [Space]
    [Space]
    [SerializeField] float rotationX;
    [SerializeField] float rotationY;
    [SerializeField] float rotationZ;

    [Tooltip("Seconds before next thumbnail is generated. Higher time gives you more time to review the image")]
    [Header("Time before next Thumbnail")]
    [Space]
    [Space]
    [Range(0.1f, 10f)] [SerializeField] float thumbnailDelay = 0.5f;

    [Tooltip("Choose Solid Color to add a custom color to your mesh prefab")]
    [Header("Change assets color if turned on")]
    [Space]
    [Space]
    [SerializeField] private bool solidColor = false;
    [SerializeField] private Material colorForPrefab;
    [SerializeField] private Color32 prefabColor;

    [Tooltip("Drag your selected assets into the Assets text")]
    [Header("Assets for Thumbnail")]
    [Space]
    [Space]
    public List<GameObject> assets = new List<GameObject>();

    [Tooltip("Where assets are spawned for thumbnail creation")]
    [Header("Asset spawn point")]
    [Space]
    [Space]
    public GameObject spawnPoint;

    [Tooltip("Set the folder in which the images will be stored. The path to the folder MUST be created beforehand " +
        "If turned OFF, images will be stored in this project Root folder.")]
    [Header("Choose Folder")]
    [Space]
    [Space]
    [SerializeField] public bool selectCustomFolder = false;
    [Tooltip("Write the path to your custom folder")]
    [SerializeField] public string folderPath;

    [Tooltip("Check to create Thumbnail. Uncheck if you want to preview every thumbnail without storing it")]
    [Header("Create Thumbnail ON/OFF. Check ToolTip")]
    [Space]
    [Space]
    [SerializeField] bool makeThumbnail = true;

    private GameObject objprefab;
    string prefabName;

    private void Awake()
    {
        myCamera = gameObject.GetComponent<Camera>();
    }

    void Update()
    {
        if (Input.GetKeyDown(KeyCode.Space))
        {
            StartCoroutine(TakeSnapshot());
        }

        EnableBackground();

        // Color for the instantiated Mesh
        colorForPrefab.SetColor("_Color", prefabColor);
        colorForPrefab.SetColor("_EmissionColor", prefabColor);


    }

    public IEnumerator TakeSnapshot()
    {
        if (Orthographic)
        {
            myCamera.orthographic = true;
        }
        else
        {
            myCamera.orthographic = false;
        }


        Vector3 pos = spawnPoint.transform.position;
        for (int i = 0; i < assets.Count; i++)
        {
            // Instantiate mesh
            objprefab = Instantiate(assets[i], pos, assets[i].transform.rotation * Quaternion.Euler(rotationX, rotationY, rotationZ));

            // Determine the color style of mesh
            if (solidColor)
            {
                MeshRenderer[] mesh = objprefab.GetComponentsInChildren<MeshRenderer>();
                for (int o = 1; o < mesh.Length; o++)
                {
                    foreach (var item in mesh)
                    {
                        item.GetComponent<MeshRenderer>().material = colorForPrefab;
                    }
                }
                if (mesh.Length <= 1)
                {
                    objprefab.GetComponent<MeshRenderer>().material = colorForPrefab;
                }
            }

            AlignCamera();

            prefabName = assets[i].name;

            if (makeThumbnail)
            {
                ImageRenderer.TakeScreenshot_Static(width, height);
            }

            yield return new WaitForSeconds(thumbnailDelay);

            Destroy(objprefab);
        }
    }

    public string PrefabName()
    {
        string pName = prefabName;
        return pName;
    }

    private void AlignCamera()
    {

        GameObject root = objprefab;

        Renderer[] renderers = root.GetComponentsInChildren<Renderer>();
        Bounds bounds = renderers[0].bounds;

        for (int i = 1; i < renderers.Length; ++i)
        {
            bounds.Encapsulate(renderers[i].bounds.min);
            bounds.Encapsulate(renderers[i].bounds.max);
            bounds.Encapsulate(renderers[i].bounds.center);
        }


        var distance = (bounds.size.magnitude + perspectiveFrameSize) * 0.5f / Mathf.Tan(myCamera.fieldOfView * 0.5f * Mathf.Deg2Rad);


        var camOrthoSize = (bounds.size.magnitude * 0.5f);
        var frameOffset = camOrthoSize * orthographicFrameSize;
        if (Orthographic)
        {
            myCamera.orthographicSize = camOrthoSize + frameOffset;
        }

        bounds = FacingDirection(bounds, distance);

        if (lookAtObject)
        {
            myCamera.transform.LookAt(bounds.center);
        }
    }

    private Bounds FacingDirection(Bounds bounds, float distance)
    {
        if (facingX)
        {
            background.transform.rotation = Quaternion.Euler(0, 0, 0);
            cameraOffset = new Vector3(-distance, 0, 0);
            myCamera.transform.position = bounds.center + cameraOffset + camOffset;
        }
        else if (facingNegX)
        {
            background.transform.rotation = Quaternion.Euler(0, 180, 0);
            cameraOffset = new Vector3(distance, 0, 0);
            myCamera.transform.position = bounds.center + cameraOffset + camOffset;
        }
        else if (facingY)
        {
            background.transform.rotation = Quaternion.Euler(0, 0, 90);
            cameraOffset = new Vector3(0, -distance, 0);
            myCamera.transform.position = bounds.center + cameraOffset + camOffset;
        }
        else if (facingNegY)
        {
            background.transform.rotation = Quaternion.Euler(0, 0, -90);
            cameraOffset = new Vector3(0, distance, 0);
            myCamera.transform.position = bounds.center + cameraOffset + camOffset;
        }
        else if (facingZ)
        {
            background.transform.rotation = Quaternion.Euler(0, -90, 0);
            cameraOffset = new Vector3(0, 0, -distance);
            myCamera.transform.position = bounds.center + cameraOffset + camOffset;
        }
        else if (facingNegZ)
        {
            background.transform.rotation = Quaternion.Euler(0, 90, 0);
            cameraOffset = new Vector3(0, 0, distance);
            myCamera.transform.position = bounds.center + cameraOffset + camOffset;
        }

        return bounds;
    }

    private bool EnableBackground()
    {
        // Color for the background
        backgroundMaterial.SetColor("_Color", backgroundColor);
        backgroundMaterial.SetColor("_EmissionColor", backgroundColor);

        if (enableBackground)
        {
            background.SetActive(true);
            return true;
        }
        background.SetActive(false);
        return false;

    }
}
