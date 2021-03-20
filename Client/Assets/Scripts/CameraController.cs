using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CameraController : MonoBehaviour
{
    private Vector3 _startPos;
    private Vector3 _startRot;
    public float LerpTime = 2f;
    public LeanTweenType TweenType;

    private LTDescr _current;

    void Start()
    {
        _startPos = transform.position;
        _startRot = transform.eulerAngles;
    }

    void Update()
    {
        if (_current != null)
            if (_current.ratioPassed > 0.99)
            {
                _current = null;
            }
        if (Input.GetMouseButtonUp(0) && _current == null)
        {
            Debug.Log("Clicked!");
            Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);
            RaycastHit hit;

            if (Physics.Raycast(ray, out hit, 100))
            {
                ClickableObject co = hit.transform.gameObject.GetComponent<ClickableObject>();

                co?.OnClick?.Invoke();
            }
        }
    }


    public void LerpToTransform(Vector3 toPos, Vector3 toRot)
    {
        _current = LeanTween.move(this.gameObject, toPos, LerpTime).setEase(TweenType);
        LeanTween.rotate(this.gameObject, toRot, LerpTime).setEase(TweenType);

    }
}