With Parts
    AS
    (
        SELECT REPLACE(P.PartName,'_','-') AS PartName,
            CASE Data1
                WHEN ''
                    THEN ''
                    ELSE 'S-' + LEFT(Data1,LEN(Data1)-1)
                END AS Job,
            CASE
                WHEN CONVERT(int,Data2) < 10
                    THEN '0' + Data2
                    ELSE Data2
            END AS Shipment, P.QtyProgram, P.TrueArea * P.QtyProgram RectArea,
                P.ProgramName, P.RepeatID, P.ArcDateTime
        FROM dbo.PartArchive P
    ),
Stocks
    AS
    (
        SELECT A.PrimeCode, A.Mill, A.ProgramName, A.RepeatID,
            CASE LEFT(A.SheetName,1)
                WHEN 'S'
                    THEN 'HS01'
                WHEN 'X'
                    THEN 'HS01'
                WHEN 'C'
                    THEN 'HS01'
                ELSE 'HS02'
            END AS Plant, A.ArcDateTime, A.Location
        FROM dbo.StockArchive A
    )

SELECT P.PartName, P.Job, P.Shipment, 'PROD' StorageLocation, SUM(P.QtyProgram) QtyProgram, 'EA' UoM_P, S.PrimeCode, S.Mill WBS_C,
SUM(P.RectArea) RectArea, 'IN2' UoM_C, S.Location, S.Plant, S.ProgramName
FROM Parts P
INNER JOIN Stocks S ON S.ProgramName = P.ProgramName AND S.RepeatID = P.RepeatID
WHERE DATEDIFF(HOUR,P.ArcDateTime,GETDATE()) BETWEEN 1 AND 4 AND DATEDIFF(HOUR, S.ArcDateTime,GETDATE()) BETWEEN 1 AND 4
GROUP BY P.PartName, P.Job, P.Shipment, S.PrimeCode, S.Mill, S.Location, S.Plant, S.ProgramName
ORDER BY S.ProgramName